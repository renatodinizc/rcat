use assert_cmd::Command;

#[test]
fn display_empty_file() {
    let mut cmd = Command::cargo_bin("rcat").unwrap();

    cmd.arg("./tests/inputs/empty.txt");
    cmd.assert().success().stdout("");
}

#[test]
fn display_phrase_file() {
    let mut cmd = Command::cargo_bin("rcat").unwrap();

    cmd.arg("./tests/inputs/phrase.txt");
    cmd.assert().success().stdout("We cannot be more sensitive to pleasure without being more sensitive to pain - Alan Watts\n");
}

#[test]
fn display_non_readable_file() {
    let mut cmd = Command::cargo_bin("rcat").unwrap();

    cmd.arg("./tests/inputs/non_readable.txt");
    cmd.assert()
        .success()
        .stderr("rcat: ./tests/inputs/non_readable.txt: Permission denied (os error 13)\n");
}

#[test]
fn display_poem_file() {
    let mut cmd = Command::cargo_bin("rcat").unwrap();

    cmd.arg("./tests/inputs/poem.txt");
    cmd.assert().success().stdout(
        "The Road Not Taken by Robert Frost

Two roads diverged in a yellow wood,
And sorry I could not travel both
And be one traveler, long I stood
And looked down one as far as I could
To where it bent in the undergrowth;

Then took the other, as just as fair,
And having perhaps the better claim,
Because it was grassy and wanted wear;
Though as for that the passing there
Had worn them really about the same,

And both that morning equally lay
In leaves no step had trodden black.
Oh, I kept the first for another day!
Yet knowing how way leads on to way,
I doubted if I should ever come back.

I shall be telling this with a sigh
Somewhere ages and ages hence:
Two roads diverged in a wood, and I—
I took the one less traveled by,
And that has made all the difference.
",
    );
}

#[test]
fn display_all_files() {
    let mut cmd = Command::cargo_bin("rcat").unwrap();

    cmd.arg("./tests/inputs/phrase.txt");
    cmd.arg("./tests/inputs/empty.txt");
    cmd.arg("./tests/inputs/non_readable.txt");
    cmd.arg("./tests/inputs/poem.txt");

    cmd.assert()
        .success()
        .stderr("rcat: ./tests/inputs/non_readable.txt: Permission denied (os error 13)\n");
    cmd.assert().success().stdout(
        "We cannot be more sensitive to pleasure without being more sensitive to pain - Alan Watts
The Road Not Taken by Robert Frost

Two roads diverged in a yellow wood,
And sorry I could not travel both
And be one traveler, long I stood
And looked down one as far as I could
To where it bent in the undergrowth;

Then took the other, as just as fair,
And having perhaps the better claim,
Because it was grassy and wanted wear;
Though as for that the passing there
Had worn them really about the same,

And both that morning equally lay
In leaves no step had trodden black.
Oh, I kept the first for another day!
Yet knowing how way leads on to way,
I doubted if I should ever come back.

I shall be telling this with a sigh
Somewhere ages and ages hence:
Two roads diverged in a wood, and I—
I took the one less traveled by,
And that has made all the difference.
",
    );
}

#[test]
fn display_from_stdin() {
    let mut cmd = Command::cargo_bin("rcat").unwrap();

    cmd.write_stdin("Pancakes and apple pie are delicious");

    cmd.assert()
        .success()
        .stdout("Pancakes and apple pie are delicious\n");
}

#[test]
fn display_file_takes_priority_over_stdin() {
    let mut cmd = Command::cargo_bin("rcat").unwrap();

    cmd.write_stdin("Pancakes and apple pie are delicious");
    cmd.arg("./tests/inputs/phrase.txt");

    cmd.assert().success().stdout("We cannot be more sensitive to pleasure without being more sensitive to pain - Alan Watts\n");
}

#[test]
fn display_file_takes_priority_over_stdin2() {
    let mut cmd = Command::cargo_bin("rcat").unwrap();

    cmd.write_stdin("Pancakes and apple pie are delicious");
    cmd.arg("./tests/inputs/empty.txt");

    cmd.assert().success().stdout("");
}

#[test]
fn display_file_takes_priority_over_stdin3() {
    let mut cmd = Command::cargo_bin("rcat").unwrap();

    cmd.write_stdin("Pancakes and apple pie are delicious");
    cmd.arg("./tests/inputs/non_readable.txt");

    cmd.assert().success().stdout("");
    cmd.assert()
        .success()
        .stderr("rcat: ./tests/inputs/non_readable.txt: Permission denied (os error 13)\n");
}

#[test]
fn display_poem_file_with_n_option() {
    let mut cmd = Command::cargo_bin("rcat").unwrap();

    cmd.arg("./tests/inputs/poem.txt");
    cmd.arg("-n");
    cmd.assert().success().stdout(
        "1 The Road Not Taken by Robert Frost
2 
3 Two roads diverged in a yellow wood,
4 And sorry I could not travel both
5 And be one traveler, long I stood
6 And looked down one as far as I could
7 To where it bent in the undergrowth;
8 
9 Then took the other, as just as fair,
10 And having perhaps the better claim,
11 Because it was grassy and wanted wear;
12 Though as for that the passing there
13 Had worn them really about the same,
14 
15 And both that morning equally lay
16 In leaves no step had trodden black.
17 Oh, I kept the first for another day!
18 Yet knowing how way leads on to way,
19 I doubted if I should ever come back.
20 
21 I shall be telling this with a sigh
22 Somewhere ages and ages hence:
23 Two roads diverged in a wood, and I—
24 I took the one less traveled by,
25 And that has made all the difference.
",
    );
}

#[test]
fn display_poem_file_with_b_option() {
    let mut cmd = Command::cargo_bin("rcat").unwrap();

    cmd.arg("./tests/inputs/poem.txt");
    cmd.arg("-b");
    cmd.assert().success().stdout(
        "1 The Road Not Taken by Robert Frost

2 Two roads diverged in a yellow wood,
3 And sorry I could not travel both
4 And be one traveler, long I stood
5 And looked down one as far as I could
6 To where it bent in the undergrowth;

7 Then took the other, as just as fair,
8 And having perhaps the better claim,
9 Because it was grassy and wanted wear;
10 Though as for that the passing there
11 Had worn them really about the same,

12 And both that morning equally lay
13 In leaves no step had trodden black.
14 Oh, I kept the first for another day!
15 Yet knowing how way leads on to way,
16 I doubted if I should ever come back.

17 I shall be telling this with a sigh
18 Somewhere ages and ages hence:
19 Two roads diverged in a wood, and I—
20 I took the one less traveled by,
21 And that has made all the difference.
",
    );
}

#[test]
fn display_poem_file_with_nb_options() {
    let mut cmd = Command::cargo_bin("rcat").unwrap();

    cmd.arg("./tests/inputs/poem.txt");
    cmd.arg("-nb");
    cmd.assert().success().stdout(
        "1 The Road Not Taken by Robert Frost

2 Two roads diverged in a yellow wood,
3 And sorry I could not travel both
4 And be one traveler, long I stood
5 And looked down one as far as I could
6 To where it bent in the undergrowth;

7 Then took the other, as just as fair,
8 And having perhaps the better claim,
9 Because it was grassy and wanted wear;
10 Though as for that the passing there
11 Had worn them really about the same,

12 And both that morning equally lay
13 In leaves no step had trodden black.
14 Oh, I kept the first for another day!
15 Yet knowing how way leads on to way,
16 I doubted if I should ever come back.

17 I shall be telling this with a sigh
18 Somewhere ages and ages hence:
19 Two roads diverged in a wood, and I—
20 I took the one less traveled by,
21 And that has made all the difference.
",
    );
}

#[test]
fn display_from_stdin_with_nb_options() {
    let mut cmd = Command::cargo_bin("rcat").unwrap();

    cmd.write_stdin(
        "Pancakes and apple pie are delicious

And the cake is a lie",
    );
    cmd.arg("-nb");

    cmd.assert().success().stdout(
        "1 Pancakes and apple pie are delicious

2 And the cake is a lie
",
    );
}
