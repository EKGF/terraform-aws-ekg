resource "null_resource" "artifact_zip" {
  triggers = {
    main      = base64sha256(file("${local.rdf_load_path}/lambda_function.py"))
    test      = base64sha256(file("${local.rdf_load_path}/lambda_function.test.py"))
    pyproject = base64sha256(file("${local.rdf_load_path}/pyproject.toml"))
    always_run = timestamp()
  }

  provisioner "local-exec" {
    command     = "./build.sh"
    interpreter = ["bash", "-c"]
    working_dir = local.rdf_load_path

    environment = {
      POETRY_BIN = "poetry"  # TODO: Make configurable
      PYTHON_BIN = "python3" # TODO: Make configurable
      ZIP_BIN    = "zip"     # TODO: Make configurable
    }
  }
}

data "archive_file" "artifact_zip" {
  depends_on       = [null_resource.artifact_zip]
  type             = "zip"
  source_dir       = "${local.rdf_load_path}/.package"
  output_path      = local.artifact_zip
  output_file_mode = "0666"
}

output "artifact_zip" {
#  value = data.local_file.artifact_zip.filename
  value = data.archive_file.artifact_zip.output_path
}

#output "artifact_log" {
#  value = data.external.execute_build.result["log_file"]
#}