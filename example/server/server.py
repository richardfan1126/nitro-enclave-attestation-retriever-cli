import socket
import json
import base64
import subprocess

from Crypto.PublicKey import RSA

def call_nsm_cli(input):
    cmd = ["/app/nsm-cli", "attest"]

    if "public-key" in input:
        cmd += ["--public-key", input["public-key"]]

    if "public-key-b64" in input:
        cmd += ["--public-key-b64", input["public-key-b64"]]

    if "user-data" in input:
        cmd += ["--user-data", input["user-data"]]

    if "user-data-b64" in input:
        cmd += ["--user-data-b64", input["user-data-b64"]]

    if "nonce" in input:
        cmd += ["--nonce", input["nonce"]]

    if "nonce-b64" in input:
        cmd += ["--nonce-b64", input["nonce-b64"]]

    # Call the standalone nsm-cli through subprocess
    proc = subprocess.Popen(cmd, stdout=subprocess.PIPE)

    result = proc.communicate()[0]

    # Return the result
    return {
        'AttestationDocument': result.decode()
    }

def generate_rsa_keypair():
    private_key = RSA.generate(2048)
    public_key = private_key.publickey()

    return private_key, public_key

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
        request = json.loads(payload.decode())

        # Generate RSA keypair
        private_key, public_key = generate_rsa_keypair()

        request['public-key-b64'] = base64.b64encode(public_key.export_key('DER')).decode()

        # Get attestation document from nsm-cli
        content = call_nsm_cli(request)

        # Send the response back to parent instance
        c.send(str.encode(json.dumps(content)))

        # Close the connection
        c.close() 

if __name__ == '__main__':
    main()
