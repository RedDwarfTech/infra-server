# infra-server

the reddwarf infrastructure service. This project replace the legacy infrastructure service written by Java, reduce the hardware resource usage.

## Generate Models

This is the steps to generate the schema file and model files:

```bash
# switch to the script folder
cd scripts/diesel
# generate the schame file
./diesel-dolphin-schema.sh
# generate the model file
./diesel-dolphin-model.sh
```

the auto generate file will be placed in `model/diesel/dolphin` folder.

