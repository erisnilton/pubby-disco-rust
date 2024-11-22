-- Add down migration script here

DELETE FROM "source"
WHERE
    "id" IN (
        '90e34016-9fd8-4c63-a238-ee079a5260da',
        'd7c6cf1d-4ccb-4c13-9f0b-cbf1c3947c45',
        '2cbb473b-9ba7-421e-91f5-7b1c95f0fb29',
        '17932738-c1c0-4620-880a-ede79bac612f',
        'bf1a1c81-0f50-462a-ace8-c8a76d99d3aa',
        '3db05ec6-0151-442b-906c-e4d2bb97b94a',
        '97a66a3a-36b4-41b4-b228-0a5ac75cc761',
        '9353cef2-f3c7-44e7-9a42-5555768e6d92',
        '7f25b5ee-13b3-4407-a408-4db2baa34f3d'
    )