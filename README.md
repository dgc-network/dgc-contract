# dgc-contract

NAMES	IMAGE                                      	PORTS                              
dgc-sde-service(dgc-sde-service)
dgc-adminer(adminer) 0.0.0.0:9003->8080/tcp
dgc-swagger-ui(dgc-swagger-ui) 0.0.0.0:9002->80/tcp
dgc-api(dgc-api) 0.0.0.0:9001->9001/tcp
dgc-client(dgc-client)
dgc-db(dgc-db) 0.0.0.0:5432->5432/tcp
dgc-tp(dgc-tp)

sabre-cli	hyperledger/sawtooth-sabre-cli             	4004/tcp, 8008/tcp                 
sabre-tp	hyperledger/sawtooth-sabre-tp              	
		
rest-api	hyperledger/sawtooth-rest-api              	4004/tcp, 0.0.0.0:8008->8008/tcp   
shell	hyperledger/sawtooth-shell                 	
settings-tp	hyperledger/sawtooth-settings-tp           	4004/tcp                           
validator	hyperledger/sawtooth-validator             	
devmode-engine	hyperledger/sawtooth-devmode-engine-rust   	

# Sawtooth Sabre

Sawtooth Sabre is a transaction family which implements on-chain smart contracts
executed in a WebAssembly virtual machine. This distributed application runs on
top of Hyperledger Sawtooth, an enterprise blockchain. To learn more about
Hyperledger Sawtooth please see the
[sawtooth-core repo](https://github.com/hyperledger/sawtooth-core) or its
[published docs](https://sawtooth.hyperledger.org/docs/).

## Documentation

Sawtooth Smart documentation can be found here:
https://sawtooth.hyperledger.org/docs/sabre/releases/latest/

## License

Hyperledger Sawtooth software is licensed under the
[Apache License Version 2.0](LICENSE) software license.
