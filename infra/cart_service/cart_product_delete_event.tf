module "cart_product_global_delete_event" {
    source = "../lambda_event_common"
    app_name = var.app_name
    lambda_name = "CartProductDeleteLambda"
    additional_policy_arns = [var.dynamo_policy_arn, var.event_bus_policy_arn]
    bootstrap_folder_name = "cart_product_global_delete_event"
    dynamo_table_name = var.dynamo_table_name
    architectures = var.architectures
    eventbridge_rule_arn = aws_cloudwatch_event_rule.cart_product_global_delete_event_rule.arn
    env_vars = {
        "EVENT_BUS_NAME" = var.event_bus_arn
    }
}

resource "aws_cloudwatch_event_rule" "cart_product_global_delete_event_rule" {
    name        = "${var.app_name}-cart_product_global_delete_event_rule"
    description = "Capture product delete events in order to remove from carts"

    event_bus_name = var.event_bus_arn

    event_pattern = jsonencode({
        detail-type = [
            "product_deleted"
        ]
    })
}

resource "aws_cloudwatch_event_target" "cart_product_global_delete_event_target" {
    rule      = aws_cloudwatch_event_rule.cart_product_global_delete_event_rule.name
    event_bus_name = var.event_bus_arn
    arn       = module.cart_product_global_delete_event.lambda_arn
    target_id = "${var.app_name}-cart_product_global_delete_event_target"
}
