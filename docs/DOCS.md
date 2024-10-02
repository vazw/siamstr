# SIAMSTR OPENED API ENDPOINT

endpoint ต่อไปนี้คือ API สำหรับเพิ่ม users เข้าไปในฐานข้อมูล

## /api/check_npub
    - Method: POST
    - Accept: JSON String
    - Payloads: public_key=<NPUB-or-HEX>
        - Ex. raw json string ไม่มี brackets  '{' '}' -> `public_key=`
    - Return JSON object: `{"user": <NULL-or-JSON>}` 
        - Ex. `{"user": {"id": string, "created": string, "name": string, "pubkey": string, "lightning_url": string}}`

### Example

#### Request:
```bash
curl -d 'public_key=npub1vaz88a5zhsqsrj220vh5vdnpjsu53msm34hzvcrh27x5d7zeav7qm45t60' https://siamstr.com/api/check_npub
```

#### Return:
```json
{
    "user": {
        "id":"4268ce77-6d58-4252-8dae-d5295e5f81f5",
        "name":"vaz",
        "pubkey":"674473f682bc0101c94a7b2f463661943948ee1b8d6e266077578d46f859eb3c",
        "lightning_url":"vazw@getalby.com",
        "created":"2024-06-03T11:05:40.186921856+00:00"
    }
}
```


## /api/check_username
    - Method: POST
    - Accept: JSON String
    - Payloads: username=string
        - Ex. raw json string ไม่มี brackets  '{' '}' -> `username=vaz`
    - Return JSON object: `{"status": <0-1>}` 
        - 0 : ใช้ได้
        - 1 : มีคนใช้แล้ว

### Example

#### Request:
```bash
curl -d 'username=vaz' https://siamstr.com/api/check_username
```

#### Return:
```json
{
    "status": 1
}
```


## /api/add_user
    - Method: POST
    - Accept: JSON String
    - Payloads: username=string&pubkey=string&lnurl=string
    - Return JSON object: `{"status": <0-1>}` 
        - 0 : failed
        - 1 : success

### Example

#### Request:
```bash
curl -d 'username=vaz&pubkey=674473f682bc0101c94a7b2f463661943948ee1b8d6e266077578d46f859eb3c&lnurl=' https://siamstr.com/api/check_username
```

#### Return:
```json
{
    "status": 1
}
```

## /api/edit_user
    - Method: POST
    - Accept: JSON String
    - Payloads: username=string&pubkey=string&lnurl=string&events=string
    - Return JSON object: `{"status": <0-1>}` 
        - 0 : failed
        - 1 : success

### Example

#### Request:
```bash
curl -d 'username=vaz&pubkey=674473f682bc0101c94a7b2f463661943948ee1b8d6e266077578d46f859eb3c&lnurl=vazw%40getalby.com&events=%7B%22id%22%3A%22fe660ce781e718d44e9fe56e1f62e6549b1422956d28364cba95a4bb76fb1aa0%22%2C%22pubkey%22%3A%22674473f682bc0101c94a7b2f463661943948ee1b8d6e266077578d46f859eb3c%22%2C%22created_at%22%3A1727864243%2C%22kind%22%3A1%2C%22tags%22%3A%5B%5D%2C%22content%22%3A%22Edit+user+siamstr.com%22%2C%22sig%22%3A%22f567a9adaba22f2aea47801cd1fa0c43bbe542b9655f6515eea587d9294d04b1474531a6631e015f669e6d8a53ca6f3ad0bd7243c8e597dbecb34e5c79e9680f%22%7D' https://siamstr.com/api/edit_user
```

#### Return:
```json
{
    "status": 1
}
```

## /api/delete_user
    - Method: POST
    - Accept: JSON String
    - Payloads: pubkey=string&events=string
    - Return JSON object: `{"status": <0-1>}` 
        - 0 : failed
        - 1 : success

### Example

#### Request:
```bash
curl -d 'pubkey=674473f682bc0101c94a7b2f463661943948ee1b8d6e266077578d46f859eb3c&events=%7B%22id%22%3A%22ca044ac6c04694fbe740e66c75832c4b8839f6c6cadc8b51e233e4a0ac67a240%22%2C%22pubkey%22%3A%22674473f682bc0101c94a7b2f463661943948ee1b8d6e266077578d46f859eb3c%22%2C%22created_at%22%3A1727864408%2C%22kind%22%3A1%2C%22tags%22%3A%5B%5D%2C%22content%22%3A%22Goodbye+siamstr.com%22%2C%22sig%22%3A%229ebe78b7bd3ef3e4255e8ee86c198c6dfd8a2b610ffd17246ce4cd6bc3d544b86500d9f70f3944c6aa768ec2f8044c57e62b71f9fd25283ba26b6863578379d4%22%7D' https://siamstr.com/api/delete_user
```

#### Return:
```json
{
    "status": 1
}
```
