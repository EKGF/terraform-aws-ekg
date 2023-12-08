resource "null_resource" "check" {
  triggers = {
    #    main       = base64sha256(file("${local.lambda_check_path}/lambda_function.py"))
    #    test       = base64sha256(file("${local.lambda_check_path}/lambda_function.test.py"))
    #    pyproject  = base64sha256(file("${local.lambda_check_path}/pyproject.toml"))
    always_run = timestamp()
  }

  provisioner "local-exec" {
    command     = "./build.sh"
    interpreter = ["bash", "-c"]
    working_dir = local.lambda_check_path

    environment = {
      POETRY_BIN = var.poetry_bin
      PYTHON_BIN = var.python_bin
      ZIP_BIN    = var.zip_bin
    }
  }
}

data "archive_file" "check" {
  depends_on       = [null_resource.check]
  type             = "zip"
  source_dir       = local.lambda_check_package_path
  output_path      = local.lambda_check_zip
  output_file_mode = "0666"
}

output "lambda_check_zip" {
  value = data.archive_file.check.output_path
}
