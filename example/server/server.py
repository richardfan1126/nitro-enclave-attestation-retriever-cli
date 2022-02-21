import socket
import json
import subprocess

def call_attestation_retriever():
    # Call the standalone nitro-enclave-attestation-retriever-cli through subprocess
    proc = subprocess.Popen(
        [
            "/app/nitro-enclave-attestation-retriever-cli",
        ],
        stdout=subprocess.PIPE
    )

    result = proc.communicate()[0]

    # Return the result
    return {
        'AttestationDocument': result.decode()
    }

def main():
    print("Starting server...")
    
    # Create a vsock socket object
    s = socket.socket(socket.AF_VSOCK, socket.SOCK_STREAM)

    # Listen for connection from any CID
    cid = socket.VMADDR_CID_ANY

    # The port should match the client running in parent EC2 instance
    port = 5000

    # Bind the socket to CID and port
    s.bind((cid, port))

    # Listen for connection from client
    s.listen()

    while True:
        c, addr = s.accept()

        # Get data sent from parent instance
        payload = c.recv(65536)

        # Get attestation document from nitro-enclave-attestation-retriever-cli
        content = call_attestation_retriever()

        # Send the response back to parent instance
        c.send(str.encode(json.dumps(content)))

        # Close the connection
        c.close() 

if __name__ == '__main__':
    main()
