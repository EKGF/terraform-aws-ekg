#!/bin/bash
#
# Build the code for the AWS Lambda.
#
# Inspired by:
#
# - https://docs.aws.amazon.com/lambda/latest/dg/python-package.html
#
POETRY_BIN=${POETRY_BIN:-poetry}
PYTHON_BIN=${PYTHON_BIN:-python3}

log_file=$(mktemp -t build.log.XXXX)
# save stdout and stderr to file descriptors 3 and 4, then redirect them to "foo"
exec 3>&1 4>&2 >${log_file} 2>&1

rm -f artifact.zip >/dev/null 2>&1 || true

#
# Build the Python code
#
echo "Building $(basename $(pwd)):"
${POETRY_BIN} build
echo "Build done"

#
# Package the Python code
#
echo "Packaging $(basename $(pwd)):"
${POETRY_BIN} run ${PYTHON_BIN} -m pip install --upgrade -t .package dist/*.whl
echo "Packaging done"

cd .package
chmod -R 644 $(find . -type f)
chmod -R 755 $(find . -type d)
find . -type d -name '*.dist-info' -exec rm -rf {} \;

# restore stdout and stderr
exec 1>&3 2>&4 3>&- 4>&-

exit 0