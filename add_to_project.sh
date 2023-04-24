# This script will handle adding the required sources to your project,
# as well as any libraries and the Cargo configuration needed to compile
# the final ELF binary

SOURCES_TO_COPY="src/serial.rs src/time.rs src/util src/pvr"

cp -r $SOURCES_TO_COPY $1/src/
cp -r .cargo $1
cp -r libs $1