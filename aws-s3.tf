resource "aws_s3_bucket" "source_data" {
  bucket              = local.prefix # ekgf-digitaltwin-dev-<name>
  object_lock_enabled = true
  force_destroy       = true
  tags                = local.default_tags
}
