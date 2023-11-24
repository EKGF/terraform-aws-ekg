#!/bin/bash
#
# Build artifact.zip for the AWS Lambda.
#
# Inspired by:
#
# - https://docs.aws.amazon.com/lambda/latest/dg/python-package.html
#
POETRY_BIN=${POETRY_BIN:-poetry}
PYTHON_BIN=${PYTHON_BIN:-python3}
ZIP_BIN=${ZIP_BIN:-zip}
JQ_BIN=${JQ_BIN:-jq}

rc=1 # we're pessimists
zip_file=""
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

#
# Zipping it up
#
echo "Zipping $(basename $(pwd)):"
cd .package
chmod -R 644 $(find . -type f)
chmod -R 755 $(find . -type d)
${ZIP_BIN} -r ../artifact.zip . -x '*__pycache__*' -x '*.pyc' -x '*.dist-info*'
cd ..
if [[ -f artifact.zip ]]; then
    zip_file="$(realpath artifact.zip)"
    echo "Zipping done: ${zip_file} $(stat -t --format=%s ${zip_file}) bytes"
    rc=0
else
    echo "Zipping failed"
fi

# restore stdout and stderr
exec 1>&3 2>&4 3>&- 4>&-

# Safely produce a JSON object containing the result value.
# jq will ensure that the value is properly quoted
# and escaped to produce a valid JSON string.
${JQ_BIN} -n --arg zip_file "${zip_file}" --arg log_file "${log_file}" '{"zip_file":$zip_file, "log_file":$log_file}'
exit ${rc}