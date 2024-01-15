

docker kill kafka
docker rm kafka

docker kill zookeeper
docker rm zookeeper

# docker pull confluentinc/cp-zookeeper:latest
# docker pull confluentinc/cp-kafka:latest

# # Run Zookeeper
# docker run -d \
#  	--net=host \
#  	--name=zookeeper \
#  	-e ZOOKEEPER_CLIENT_PORT=32181 \
#  	-e ZOOKEEPER_TICK_TIME=2000 \
#  	confluentinc/cp-zookeeper:3.0.0
# # Run Kafka Broker
#  docker run -d \
#  	--net=host \
#  	--name=kafka \
#  	-e KAFKA_ZOOKEEPER_CONNECT=localhost:32181 \
#  	-e KAFKA_ADVERTISED_LISTENERS=PLAINTEXT://localhost:9191 \
#  	confluentinc/cp-kafka:3.0.0

docker pull wurstmeister/kafka:latest
docker pull  wurstmeister/zookeeper:latest

# Step 2: Run Kafka and Zookeeper Containers
docker run -d \
  --name zookeeper \
  -p 2181:2181 \
  -p 2888:2888 \
  -p 3888:3888 \
  wurstmeister/zookeeper:latest

docker run -d \
  --name kafka \
  -p 9092:9092 \
  --link zookeeper:zookeeper \
  -e KAFKA_ADVERTISED_LISTENERS=PLAINTEXT://localhost:9092 \
    -e KAFKA_LISTENERS=PLAINTEXT://0.0.0.0:9092 \
  -e KAFKA_ZOOKEEPER_CONNECT=zookeeper:2181 \
  wurstmeister/kafka:latest
