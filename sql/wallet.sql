/*
 Navicat Premium Data Transfer

 Source Server         : localhost
 Source Server Type    : PostgreSQL
 Source Server Version : 90524
 Source Host           : localhost:5432
 Source Catalog        : postgres
 Source Schema         : wallet

 Target Server Type    : PostgreSQL
 Target Server Version : 90524
 File Encoding         : 65001

 Date: 14/02/2021 21:33:35
*/


-- ----------------------------
-- Table structure for settle
-- ----------------------------
DROP TABLE IF EXISTS "public"."settle";
CREATE TABLE "public"."settle" (
  "id" varchar(50) COLLATE "pg_catalog"."default" NOT NULL,
  "type" varchar(255) COLLATE "pg_catalog"."default",
  "channel" varchar(255) COLLATE "pg_catalog"."default",
  "recipient" json,
  "wallet_id" varchar(50) COLLATE "pg_catalog"."default",
  "extra" json,
  "created" int8
)
;
ALTER TABLE "public"."settle" OWNER TO "postgres";
COMMENT ON COLUMN "public"."settle"."id" IS '对象id
';
COMMENT ON COLUMN "public"."settle"."type" IS '值为settle，表示此对象为支付对象
';
COMMENT ON COLUMN "public"."settle"."channel" IS '渠道，当前仅支持银行卡（bank_card）
';
COMMENT ON COLUMN "public"."settle"."recipient" IS '结算账号信息
';
COMMENT ON COLUMN "public"."settle"."wallet_id" IS '钱包对象id，即结算对象相关关联的钱包对象
';
COMMENT ON COLUMN "public"."settle"."extra" IS '额外参数
';
COMMENT ON COLUMN "public"."settle"."created" IS '创建结算对象时间';

-- ----------------------------
-- Table structure for wallet
-- ----------------------------
DROP TABLE IF EXISTS "public"."wallet";
CREATE TABLE "public"."wallet" (
  "id" varchar(50) COLLATE "pg_catalog"."default" NOT NULL,
  "type" varchar(255) COLLATE "pg_catalog"."default",
  "appid" varchar(50) COLLATE "pg_catalog"."default",
  "extra" json,
  "block" bit(1),
  "encrypted_password" varchar(255) COLLATE "pg_catalog"."default",
  "bank_cards" json,
  "reviewed" bit(1),
  "available_balance" int8,
  "withdrawable_balance" int8,
  "freeze_balance" int8,
  "channel" varchar(255) COLLATE "pg_catalog"."default",
  "created" int8
)
;
ALTER TABLE "public"."wallet" OWNER TO "postgres";
COMMENT ON COLUMN "public"."wallet"."id" IS '对象id';
COMMENT ON COLUMN "public"."wallet"."type" IS '值为wallet，表示此对象为支付对象';
COMMENT ON COLUMN "public"."wallet"."appid" IS '对应 app 对象的 id';
COMMENT ON COLUMN "public"."wallet"."extra" IS '扩展用户字段
';
COMMENT ON COLUMN "public"."wallet"."block" IS '是否锁定
';
COMMENT ON COLUMN "public"."wallet"."encrypted_password" IS '加密后的支付密码
';
COMMENT ON COLUMN "public"."wallet"."bank_cards" IS '记录银行卡对象
';
COMMENT ON COLUMN "public"."wallet"."reviewed" IS '是否经过审核
';
COMMENT ON COLUMN "public"."wallet"."available_balance" IS '可用余额，可用于消费。
';
COMMENT ON COLUMN "public"."wallet"."withdrawable_balance" IS '可提现余额，可用于消费、提现、转账等。
';
COMMENT ON COLUMN "public"."wallet"."freeze_balance" IS '冻结金额。
';
COMMENT ON COLUMN "public"."wallet"."channel" IS '渠道，对应钱包账户服务商的渠道。
';
COMMENT ON COLUMN "public"."wallet"."created" IS '钱包unix时间戳';

-- ----------------------------
-- Primary Key structure for table settle
-- ----------------------------
ALTER TABLE "public"."settle" ADD CONSTRAINT "settle_pkey" PRIMARY KEY ("id");

-- ----------------------------
-- Primary Key structure for table wallet
-- ----------------------------
ALTER TABLE "public"."wallet" ADD CONSTRAINT "wallet_pkey" PRIMARY KEY ("id");
