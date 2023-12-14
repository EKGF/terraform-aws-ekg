resource "null_resource" "load" {
  triggers = {
    #    main       = base64sha256(file("${local.lambda_load_path}/lambda_function.py"))
    #    test       = base64sha256(file("${local.lambda_load_path}/lambda_function.test.py"))
    #    pyproject  = base64sha256(file("${local.lambda_load_path}/pyproject.toml"))
    always_run = timestamp()
  }

  provisioner "local-exec" {
    command     = "./build.sh"
    interpreter = ["bash", "-c"]
    working_dir = local.lambda_load_path

    environment = {
      ARTIFACT_ZIP = local.lambda_load_zip
      POETRY_BIN   = var.poetry_bin
      PYTHON_BIN   = var.python_bin
      ZIP_BIN      = var.zip_bin
    }
  }
}

data "archive_file" "load" {
  depends_on       = [null_resource.load]
  type             = "zip"
  source_dir       = local.lambda_load_package_path
  output_path      = local.lambda_load_zip
  output_file_mode = "0666"
}

output "lambda_load_zip" {
  value = data.archive_file.load.output_path
}
