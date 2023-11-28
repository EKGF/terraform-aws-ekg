variable "org_short" {
  type        = string
  description = "Short organization name"
}

variable "project_short" {
  type        = string
  description = "Project id (short code)"
}

variable "aws_region" {
  type        = string
  description = "AWS Region"
  default     = "eu-west-2" # London, make sure that this region is also in the list of availability zones
}

variable "aws_account_id" {
  type        = string
  description = "AWS Account ID"
}

variable "aws_access_key_id" {
  type        = string
  description = "AWS Access Key ID, passed in via terraform environment variable TF_VAR_aws_access_key_id"
  sensitive   = true
}

variable "aws_secret_access_key" {
  type        = string
  description = "AWS Secret Access Key, passed in via terraform environment variable TF_VAR_aws_secret_access_key"
  sensitive   = true
}

variable "environment" {
  type        = string
  description = "Environment (dev / stage / prod)"
}

variable "name" {
  type        = string
  description = "Name of loader (for instance 'metadata' or 'staging') which is used in all resources"
}

variable "enable_sns_cloudwatch" {
  default     = true
  type        = bool
  description = "Boolean flag that determines if we switch on logging of all SNS topics to Cloudwatch"
}

variable "iam_permissions_boundary" {
  type        = string
  description = "IAM permissions boundary policy name (ie the part after '/policy/')"
  default     = null
}

variable "vpc_name" {
  type        = string
  description = "base of names for the deployed VPC assets (mandatory, even if vpc_id is not set)"
  default     = "digital-twin"
}

variable "sparql_update_endpoint_host" {
  type        = string
  description = "SPARQL endpoint URL"
  default     = null
}

variable "sparql_update_endpoint_port" {
  type        = number
  description = "SPARQL endpoint port"
  default     = null
}

variable "neptune_s3_iam_role_arn" {
  type        = string
  description = "ARN of the IAM role that Neptune uses to access S3"
  default     = null
}

variable "neptune_cluster_subnet_ids" {
  type        = list(string)
  description = "List of subnet IDs for Neptune cluster"
  default     = null
}

variable "neptune_cluster_security_group_ids" {
  type        = set(string)
  description = "Set of security group IDs for Neptune cluster"
  default     = null
}

variable "tags" {
  type        = map(string)
  description = "Tags to apply to all resources"
  default     = {
    "org_short"   = "ekgf",
    "project"     = "dt"
    "environment" = "dev"
  }
}

variable "ekg_base_placeholder" {
  description = "The placeholder for the EKG base URL"
  type        = string
  default     = "https://placeholder.kg"
}

variable "ekg_id_base_placeholder" {
  description = "The base placeholder URL for EKG IDs"
  type        = string
  default     = "https://placeholder.kg/id"
}

variable "ekg_graph_base_placeholder" {
  description = "The base placeholder URL for EKG Graph IDs"
  type        = string
  default     = "https://placeholder.kg/graph"
}

variable "ekg_base_internal" {
  description = "The placeholder for the EKG base URL"
  type        = string
  default     = "https://placeholder.kg"
}

variable "ekg_id_base_internal" {
  description = "The base placeholder URL for EKG IDs"
  type        = string
  default     = "https://placeholder.kg/id"
}

variable "ekg_graph_base_internal" {
  description = "The base placeholder URL for EKG Graph IDs"
  type        = string
  default     = "https://placeholder.kg/graph"
}

variable "ekg_ontology_base_internal" {
  description = "The base placeholder URL for the organization's own Ontology IRIs"
  type        = string
  default     = "https://placeholder.kg/ontology"
}

variable "ekg_base_external" {
  description = "The EKG base URL"
  type        = string
}

variable "ekg_id_base_external" {
  description = "The base URL for EKG IDs"
  type        = string
}

variable "ekg_graph_base_external" {
  description = "The base URL for EKG Graph IDs"
  type        = string
}

variable "ekg_ontology_base_external" {
  description = "The base URL for the organization's own Ontology IRIs"
  type        = string
}

variable "ekg_api_base" {
  description = "The base URL for the EKG API"
  type        = string
}

variable "ekg_sparql_health_endpoint" {
  description = "The URL for the SPARQL health endpoint, used to check for a 200 status"
  type        = string
}

variable "ekg_sparql_query_endpoint" {
  description = "The URL for the SPARQL query endpoint (used for ASK, SELECT or CONSTRUCT statements)"
  type        = string
}

variable "ekg_sparql_update_endpoint" {
  description = "The URL for the SPARQL update endpoint (used for INSERT/UPDATE/DELETE, DROP, or LOAD)"
  type        = string
}
