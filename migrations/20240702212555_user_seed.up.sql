-- Habilita a extensão UUID

INSERT INTO
    "users" (
        "id",
        "username",
        "password",
        "display_name",
        "email",
        "is_curator"
    )
VALUES (
        'b5fc35ba-0990-482b-8988-6b25f25f9052',
        'erisnilton',
        '$2b$10$6rT1/kS7Vf0LCt2u1ElXUe9apiX/zIAEuT9WalcbK.Te5kbGNqDsm',
        'Erisnilton',
        'contato@erisnilton.dev',
        false
    ),
    (
        '8bcf473d-3744-4ad5-b3eb-3d9c49979fd6',
        'sallon',
        '$2b$10$8jFC/7a3fGSNa4eY4TGlFOY4Q/g7CB6peIdnZE01wo4b4UFXpA96C',
        'Salomão Neto',
        'contato@sallon.dev',
        true
    )