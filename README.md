# axum-surreal-restful-template
restful api with axum and surrealdb template

# surreal db docker

create directory for data store
```bash
mkdir tmp
```

create network
```bash
docker network create dev-network
```

create surrealdb container
```bash
docker run --name surreal_template --network dev-network --rm --pull always -p 8000:8000 -v  ./tmp:/container-dir surrealdb/surrealdb:latest start --auth --user root --pass root file:/container-dir/template.db
```

connect to surrealdb
```bash
surreal sql --endpoint http://localhost:8000 --username root --password root --namespace ns_template --database db_template
```