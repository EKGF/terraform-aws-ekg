resource "null_resource" "load" {
  triggers = {
    always_run = timestamp()
  }

  provisioner "local-exec" {
    command     = "cargo lambda build --release --bin ${local.lambda_load_crate} --arm64 --output-format binary"
    working_dir = local.lambda_load_crate_path
  }
}

data "archive_file" "load" {
  depends_on       = [null_resource.load]
  type             = "zip"
  source_dir       = local.lambda_load_package_path
  output_path      = local.lambda_load_zip
  output_file_mode = "0666"
  excludes         = setunion(
    fileset(local.lambda_load_package_path, "**/*.zip"),
    [local.lambda_load_zip]
  )
}

output "lambda_load_zip" {
  value = data.archive_file.load.output_path
}
