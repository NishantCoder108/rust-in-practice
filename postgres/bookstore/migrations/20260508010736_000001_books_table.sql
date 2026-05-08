create table book (
    title varchar not null,
    author varchar not null,
    isbn varchar not null
);


-- This statement creates a unique index named 'book_isbn_idx' on the 'isbn' column of the 'book' table.
-- The unique index ensures that each value in the 'isbn' column is distinct, preventing duplicate ISBNs in the table.
create unique index book_isbn_idx on book (isbn);