#
# See https://registry.terraform.io/providers/hashicorp/aws/latest/docs/resources/s3_bucket_notification
#
# Enables S3 to publish events to SNS, every time a new object is created or updated in the various RDF buckets
#
data "aws_iam_policy_document" "s3_to_sns" {
  statement {
    sid       = "Allow S3 to publish RDF load events to SNS"
    effect    = "Allow"
    actions   = ["SNS:Publish"]
    resources = ["arn:aws:sns:*:*:${local.full_name}"]

    principals {
      type        = "Service"
      identifiers = [
        "s3.amazonaws.com",
        "events.amazonaws.com",
        "cloudtrail.amazonaws.com"
      ]
    }

    condition {
      test     = "ArnLike"
      variable = "aws:SourceArn"
      values   = [
        aws_s3_bucket.source_data.arn
      ]
    }
  }
}


