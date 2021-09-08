CREATE TABLE "public"."idp_clients" (
    "id" text NOT NULL,
    "created_at" timestamp(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" timestamp(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "name" text NOT NULL,
    "redirect_uris" text[],
    PRIMARY KEY ("id")
);

CREATE TABLE "public"."invitations" (
    "id" uuid DEFAULT uuid_generate_v4 (),
    "created_at" timestamp(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" timestamp(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "email" text NOT NULL UNIQUE,
    "code" text NOT NULL UNIQUE,
    "used_at" timestamp(3),
    "redirect_uri" text NOT NULL,
    "idp_client_id" text NOT NULL,
    PRIMARY KEY ("id")
);

CREATE TABLE "public"."password_resets" (
    "id" uuid DEFAULT uuid_generate_v4 (),
    "created_at" timestamp(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" timestamp(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "user_id" uuid NOT NULL UNIQUE,
    "code" text NOT NULL,
    "used_at" timestamp(3),
    "redirect_uri" text NOT NULL,
    "idp_client_id" text NOT NULL,
    PRIMARY KEY ("id")
);

CREATE TABLE "public"."users" (
    "id" uuid DEFAULT uuid_generate_v4 (),
    "created_at" timestamp(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" timestamp(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "email" text NOT NULL UNIQUE,
    "encrypted_password" text NOT NULL,
    "terms_accepted_at" timestamp(3),
    PRIMARY KEY ("id")
);

ALTER TABLE "public"."invitations" ADD FOREIGN KEY ("idp_client_id") REFERENCES "public"."idp_clients"("id") ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE "public"."password_resets" ADD FOREIGN KEY ("user_id") REFERENCES "public"."users"("id") ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE "public"."password_resets" ADD FOREIGN KEY ("idp_client_id") REFERENCES "public"."idp_clients"("id") ON DELETE CASCADE ON UPDATE CASCADE;

SELECT manage_updated_at('idp_clients');
SELECT manage_updated_at('invitations');
SELECT manage_updated_at('password_resets');
SELECT manage_updated_at('users');
