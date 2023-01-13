CREATE TABLE IF NOT EXISTS spider_nft_collections_trending (
	`collection_id` varchar(256),
	`slug` varchar(256),
	`num_tokens` decimal(50, 0),
	`creator` varchar(256),
	`name` varchar(256),
	`description` text,
	`max_amount` decimal(50, 0),
	`uri` varchar(256),
	`logo_uri` varchar(256),
	`verified` tinyint(1),
	`total_volume` decimal(50, 0),
	`floor` decimal(50, 0),
	`num_unique_owners` decimal(50, 0),
	`volume_24` decimal(50, 0),
	`volume_48` decimal(50, 0),
	`floor_24` decimal(50, 0),
	`create_time` datetime NOT NULL DEFAULT now() COMMENT '创建时间',
	`update_time` datetime NOT NULL DEFAULT now() COMMENT '更新时间',
	`delete_time` datetime DEFAULT NULL COMMENT '删除时间',
	PRIMARY KEY (`collection_id`)
)
ENGINE=InnoDB
DEFAULT CHARSET=utf8mb4
COLLATE=utf8mb4_0900_ai_ci;
