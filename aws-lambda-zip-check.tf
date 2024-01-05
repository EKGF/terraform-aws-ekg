resource "null_resource" "check" {
  triggers = {
    always_run = timestamp()
  }

  provisioner "local-exec" {
    command     = "cargo lambda build --release --bin ${local.lambda_check_crate} --arm64 --output-format binary"
    working_dir = local.lambda_check_crate_path
  }
}

data "archive_file" "check" {
  depends_on       = [null_resource.check]
  type             = "zip"
  source_dir       = local.lambda_check_package_path
  output_path      = local.lambda_check_zip
  output_file_mode = "0666"
  excludes         = setunion(
    fileset(local.lambda_check_package_path, "**/*.zip"),
    [local.lambda_check_zip]
  )
}

output "lambda_check_zip" {
  value = data.archive_file.check.output_path
}
