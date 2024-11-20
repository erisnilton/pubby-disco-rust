-- Add down migration script here

DELETE FROM "media" CASCADE
WHERE
    "id" IN (
        '79e1cb2a-a865-4d45-92e7-259c5c94c07e',
        'fc460aed-9865-49d8-91aa-1965cd4b0d18',
        'b3747258-b1f6-4a4c-801d-3f38e2baddd1',
        '51d5916d-e6ec-4239-8873-0d60daa5f88d',
        'c9ceceb8-97a6-404f-bced-f7116cf5db25',
        '6ff9fe67-2f05-4e1e-a105-6425b72ff4cc',
        '50276085-9780-4352-8ad7-bde738c8de31',
        '73125191-81d0-4e33-adaa-eba8393a1d55',
        '3ebcc6f3-9a55-4fd2-ac3f-827e42a384eb'
    );