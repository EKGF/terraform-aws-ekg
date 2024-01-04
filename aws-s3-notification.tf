resource "aws_s3_bucket_notification" "s3_notif" {
  provider = aws.ekg_api
  bucket   = aws_s3_bucket.source_data.id

  topic {
    topic_arn = aws_sns_topic.rdf_load.arn

    # See https://docs.aws.amazon.com/AmazonS3/latest/userguide/notification-how-to-event-types-and-destinations.html
    events = [
      "s3:ObjectCreated:*",
      "s3:ObjectTagging:*"
    ]
  }

  depends_on = [
    aws_sns_topic.rdf_load
  ]
}

