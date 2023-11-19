resource "null_resource" "artifact_zip" {
  triggers = {
    main      = base64sha256(file("${local.rdf_load_path}/lambda_function.py"))
    test      = base64sha256(file("${local.rdf_load_path}/lambda_function.test.py"))
    pyproject = base64sha256(file("${local.rdf_load_path}/pyproject.toml"))
    #    always_run = "${timestamp()}"
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

#data "external" "execute_build" {
#  program     = ["/usr/local/bin/bash", "-c", "./build.sh"]
#  working_dir = local.rdf_load_path
#}

data "archive_file" "artifact_zip" {
  depends_on       = [null_resource.artifact_zip]
  #  depends_on  = [data.external.execute_build]
  type             = "zip"
  source_dir       = local.rdf_load_path
  #    source_dir  = "${path.module}/lambda/rdf-load"
  output_path      = "${path.module}/lambda/rdf-load/artifact.zip"
  #  output_path = data.external.execute_build.result["zip_file"]
  output_file_mode = "0666"
}

output "artifact_zip" {
  value = data.archive_file.artifact_zip.output_path
}

#output "artifact_log" {
#  value = data.external.execute_build.result["log_file"]
#}