resource "aws_s3_bucket_logging" "logging" {
  bucket        = aws_s3_bucket.source_data.id
  target_bucket = aws_s3_bucket.log_bucket.id
  target_prefix = "log/"
}

resource "aws_s3_bucket" "log_bucket" {
  bucket = "${local.prefix}-bucket-log"
}


