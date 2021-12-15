CREATE TABLE users (
  "id" int GENERATED always AS IDENTITY PRIMARY KEY,
  "gh_username" text NOT NULL,
  "gh_token" text NOT NULL,
  "created_at" timestamp WITH time zone NOT NULL DEFAULT now(),
  "updated_at" timestamp WITH time zone NOT NULL DEFAULT now()
);

CREATE TABLE profiles (
  "id" int GENERATED always AS IDENTITY PRIMARY KEY,
  "user_id" int NOT NULL REFERENCES users (id) ON DELETE RESTRICT,
  "name" text NOT NULL,
  "bio" text NOT NULL,
  "avatar_url" text NOT NULL,
  "public_repos" int NOT NULL DEFAULT 0,
  "public_gists" int NOT NULL DEFAULT 0,
  "created_at" timestamp WITH time zone NOT NULL DEFAULT now(),
  "updated_at" timestamp WITH time zone NOT NULL DEFAULT now()
);