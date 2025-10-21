-- Add migration script here

CREATE TABLE "website" (
    "id" TEXT NOT NULL,
    "url" TEXT NOT NULL,
    "time_added" TIMESTAMP(3) NOT NULL,
    CONSTRAINT "website_pkey" PRIMARY KEY ("id")
);

