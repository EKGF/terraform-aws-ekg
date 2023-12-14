
.PHONY: clean
clean:
	-@rm -rf .package dist .pytest_cache

.PHONY: install
install:
	poetry install

.PHONY: test
test: install
	poetry run python lambda_function.test.py

.PHONE: build
build: install
	poetry build

.PHONY: package
package: build
	poetry run python -m pip install -v --upgrade -t .package dist/*.whl
	rm -rf .package/*.dist-info .package/*.egg-info


