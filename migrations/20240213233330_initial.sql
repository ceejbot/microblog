-- sqlx will run migrations for us

CREATE TABLE statuses (
	id text primary key,
	body text NOT NULL,
	created timestamptz default now() NOT NULL,
	modified timestamptz default now() NOT NULL,
	deleted timestamptz
);
