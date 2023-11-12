resource "aws_s3_bucket_logging" "logging" {
  bucket        = aws_s3_bucket.source_data.id
  target_bucket = data.aws_s3_bucket.log.id
  target_prefix = "log/${local.prefix}-bucket/"
}

data "aws_s3_bucket" "log" {
  bucket = "${local.stack}-log"
}


