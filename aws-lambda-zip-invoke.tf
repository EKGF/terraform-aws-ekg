resource "null_resource" "invoke" {
  triggers = {
    always_run = timestamp()
  }

  provisioner "local-exec" {
    command     = "cargo lambda build --release --bin ${local.lambda_invoke_crate} --arm64 --output-format binary"
    working_dir = local.lambda_invoke_crate_path
  }
}

data "archive_file" "invoke" {
  depends_on       = [null_resource.invoke]
  type             = "zip"
  source_dir       = local.lambda_invoke_package_path
  output_path      = local.lambda_invoke_zip
  output_file_mode = "0666"
  excludes         = setunion(
    fileset(local.lambda_invoke_package_path, "**/*.zip"),
    [local.lambda_invoke_zip]
  )
}

output "lambda_invoke_zip" {
  value = data.archive_file.invoke.output_path
}