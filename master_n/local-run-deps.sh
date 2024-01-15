

docker kill kafka
docker rm kafka

docker kill zookeeper
docker rm zookeeper

docker pull confluentinc/cp-zookeeper:latest
docker pull confluentinc/cp-kafka:latest

# Run Zookeeper
docker run -d \
 	--net=host \
 	--name=zookeeper \
 	-e ZOOKEEPER_CLIENT_PORT=32181 \
 	-e ZOOKEEPER_TICK_TIME=2000 \
 	confluentinc/cp-zookeeper:3.0.0
# Run Kafka Broker
 docker run -d \
 	--net=host \
 	--name=kafka \
 	-e KAFKA_ZOOKEEPER_CONNECT=localhost:32181 \
 	-e KAFKA_ADVERTISED_LISTENERS=PLAINTEXT://localhost:9191 \
 	confluentinc/cp-kafka:3.0.0