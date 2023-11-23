#!/bin/bash
#
# Build artifact.zip for the AWS Lambda.
#

log_file=$(mktemp -t build.log.XXXX)
# save stdout and stderr to file descriptors 3 and 4, then redirect them to "foo"
exec 3>&1 4>&2 >${log_file} 2>&1

echo "Building..."
POETRY_BIN=poetry
PYTHON_BIN=python3
ZIP_BIN=zip
JQ_BIN=jq
rm -f artifact.zip >/dev/null 2>&1 || true
${POETRY_BIN} build
echo "Build done"
${POETRY_BIN} run ${PYTHON_BIN} -m pip install --upgrade -t .package dist/*.whl
cd .package
chmod -R 644 $(find . -type f)
chmod -R 755 $(find . -type d)
ls -al
echo "Zipping it up:"
${ZIP_BIN} -r ../artifact.zip . -x '*.pyc'
cd ..
ls -al artifact.zip
zip_file="$(realpath artifact.zip)"

# restore stdout and stderr
exec 1>&3 2>&4 3>&- 4>&-

# Safely produce a JSON object containing the result value.
# jq will ensure that the value is properly quoted
# and escaped to produce a valid JSON string.
${JQ_BIN} -n --arg zip_file "${zip_file}" --arg log_file "${log_file}" '{"zip_file":$zip_file, "log_file":$log_file}'
exit 0