# create the db and run migrations
createdb:
	createdb microblog
	DATABASE_URL="postgres://localhost/microblog" sqlx migrate run

# drop that db like a hot potato
dropdb:
	dropdb microblog
