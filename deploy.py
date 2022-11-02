import cwpy
import requests
import yaml, json

def get_umlg_from_faucet(addr):
    data = {"denom":"umlg", "address":addr} 
    r = requests.post("https://faucet.malaga-420.cosmwasm.com/credit", json=data)
    if "ok" in r.text:
        return True
    else:
        return False

def code_id_from_log(log):
    logs = json.loads(yaml.load(log, Loader=yaml.FullLoader)['raw_log'])[0]
    code_id = logs["events"][-1]["attributes"][0]["value"]
    return int(code_id)
    
def store(_filename):
    filename = _filename
    with open(filename, "rb") as f:
        wasmBytes = f.read()
    res = w.tx_wasm_store("wooz3k", wasmBytes)
    code_id = code_id_from_log(res)
    return code_id

def instantiate(_imsg, code_id):
    imsg = _imsg
    res = w.tx_wasm_instantiate("wooz3k", code_id, "wooz3k", imsg, 100)
    logs = json.loads(yaml.load(res, Loader=yaml.FullLoader)['raw_log'])[0]
    for e in logs['events']:
        if e["type"] == "instantiate":
            o = { x["key"]:x["value"] for x in e["attributes"]}
            assert(int(o["code_id"]) == code_id)
            return o["_contract_address"]

def execute(_msg, contract_addr):
    msg = _msg
    res = w.tx_wasm_execute("wooz3k", contract_addr, msg, 100000)
    print(res)

def query_smart(qry, contract_addr):
    res = w.query_contract_state_smart(contract_addr, qry)
    print(res)

if __name__ == "__main__":
    w = cwpy.wallet("malaga-420", "https://rpc.malaga-420.cosmwasm.com:443")
    mnemonic = "praise convince exercise deputy rookie disagree grocery leopard sure tumble bronze cave text spend box chalk antique music slight vendor require rib host pool"
    w.add_key_with_mnemonic("wooz3k", mnemonic)
    addr = w.get_key("wooz3k")
    #print(addr)
    get_umlg_from_faucet(addr)

    path = "/mnt/c/Users/wooz3k/Desktop/cosmwasm/test_contract/flow-test-2/artifacts/"

    _filename = path + "flow_test_2.wasm"
    submsg_code_id = store(_filename)
    init_test = b"{}"
    submsg_addr = instantiate(init_test, submsg_code_id)
    print(submsg_addr)
    
    a = json.dumps({"flow":{}})
    execute(a, submsg_addr)