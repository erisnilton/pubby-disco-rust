INSERT INTO
    "album" (
        "id",
        "name",
        "cover",
        "slug",
        "release_date",
        "parental_rating"
    )
VALUES (
        'b9d5b052-06a9-4af7-8d3f-80f4c3b3c47a',
        'Thriller',
        'https://example.com/album/thriller.jpg',
        'thriller',
        '1982-11-30',
        12
    );

INSERT INTO
    "album_artist" ("album_id", "artist_id")
VALUES (
        'b9d5b052-06a9-4af7-8d3f-80f4c3b3c47a',
        '4d171119-0ff4-4fd2-b75e-614f20221bbe'
    )