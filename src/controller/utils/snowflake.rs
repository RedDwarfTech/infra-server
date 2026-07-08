use anyhow::{Error, Result};
use bigdecimal::ToPrimitive;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// 开始时间戳（2022-08-01）
const TWEPOCH: i64 = 1659283200000;
// 机器id所占的位数
const WORKER_ID_BITS: i64 = 5;
// 数据节点所占的位数
const DATA_CENTER_ID_BITS: i64 = 5;
// 支持最大的机器ID，最大是31
const MAX_WORKER_ID: i64 = (-1 ^ (-1 << WORKER_ID_BITS)) as i64;
// 支持的最大数据节点ID，结果是31
const MAX_DATA_CENTER_ID: i64 = (-1 ^ (-1 << DATA_CENTER_ID_BITS)) as i64;
// 序列号所占的位数
const SEQUENCE_BITS: i64 = 12;
// 工作节点标识ID向左移12位
const WORKER_ID_SHIFT: i64 = SEQUENCE_BITS;
// 数据节点标识ID向左移动17位（12位序列号+5位工作节点）
const DATA_CENTER_ID_SHIFT: i64 = SEQUENCE_BITS + WORKER_ID_BITS;
// 时间戳向左移动22位（12位序列号+5位工作节点+5位数据节点）
const TIMESTAMP_LEFT_SHIFT: i64 = SEQUENCE_BITS + WORKER_ID_BITS + DATA_CENTER_ID_BITS;
// 生成的序列掩码，这里是4095
const SEQUENCE_MASK: i64 = (-1 ^ (-1 << SEQUENCE_BITS)) as i64;
// NTP fine adjustments are typically within a few milliseconds
const MAX_BACKWARD_MS: i64 = 5;
// Maximum wall-clock lag before refusing to generate IDs
const MAX_LOGICAL_DRIFT_MS: i64 = 30_000;

#[derive(Clone)]
pub struct SnowflakeIdWorker(Arc<Mutex<SnowflakeIdWorkerInner>>);
impl SnowflakeIdWorker {
    pub fn new(worker_id: i64, data_center_id: i64) -> Result<SnowflakeIdWorker> {
        Ok(Self(Arc::new(Mutex::new(SnowflakeIdWorkerInner::new(
            worker_id,
            data_center_id,
        )?))))
    }
    pub fn next_id(&self) -> Result<i64> {
        let mut inner = self.0.lock().map_err(|e| Error::msg(e.to_string()))?;
        inner.next_id()
    }
}

struct SnowflakeIdWorkerInner {
    worker_id: i64,
    data_center_id: i64,
    sequence: i64,
    last_timestamp: i64,
}

impl SnowflakeIdWorkerInner {
    fn new(worker_id: i64, data_center_id: i64) -> Result<SnowflakeIdWorkerInner> {
        if worker_id > MAX_WORKER_ID {
            return Err(Error::msg(format!(
                "workerId:{} must be less than {}",
                worker_id, MAX_WORKER_ID
            )));
        }
        if data_center_id > MAX_DATA_CENTER_ID {
            return Err(Error::msg(format!(
                "datacenterId:{} must be less than {}",
                data_center_id, MAX_DATA_CENTER_ID
            )));
        }
        Ok(SnowflakeIdWorkerInner {
            worker_id,
            data_center_id,
            sequence: 0,
            last_timestamp: 0,
        })
    }

    fn next_id(&mut self) -> Result<i64> {
        let wall_ts = Self::get_time()?;
        let mut timestamp = self.resolve_timestamp(wall_ts)?;

        if timestamp == self.last_timestamp {
            self.sequence = (self.sequence + 1) & SEQUENCE_MASK;
            if self.sequence == 0 {
                timestamp = self.advance_on_sequence_overflow()?;
            }
        } else {
            self.sequence = 0;
        }

        self.last_timestamp = timestamp;
        Ok(((timestamp - TWEPOCH) << TIMESTAMP_LEFT_SHIFT)
            | (self.data_center_id << DATA_CENTER_ID_SHIFT)
            | (self.worker_id << WORKER_ID_SHIFT)
            | self.sequence)
    }

    fn resolve_timestamp(&self, wall_ts: i64) -> Result<i64> {
        if wall_ts >= self.last_timestamp {
            return Ok(wall_ts);
        }

        let offset = self.last_timestamp - wall_ts;
        if offset <= MAX_BACKWARD_MS {
            return Self::til_next_mills(self.last_timestamp);
        }
        if offset <= MAX_LOGICAL_DRIFT_MS {
            log::warn!(
                "Clock moved backwards {}ms, using logical timestamp",
                offset
            );
            return Ok(self.last_timestamp);
        }

        Err(Error::msg(format!(
            "Clock moved backwards {}ms, exceeds max logical drift {}ms",
            offset, MAX_LOGICAL_DRIFT_MS
        )))
    }

    fn advance_on_sequence_overflow(&self) -> Result<i64> {
        if Self::get_time()? > self.last_timestamp {
            Self::til_next_mills(self.last_timestamp)
        } else {
            Ok(self.last_timestamp + 1)
        }
    }

    fn til_next_mills(last_timestamp: i64) -> Result<i64> {
        let mut timestamp = Self::get_time()?;
        while timestamp <= last_timestamp {
            std::thread::sleep(Duration::from_micros(100));
            timestamp = Self::get_time()?;
        }
        Ok(timestamp)
    }

    fn get_time() -> Result<i64> {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(s) => Ok(s.as_millis().to_i64().unwrap()),
            Err(_) => Err(Error::msg("get_time error!")),
        }
    }
}
