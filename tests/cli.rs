use assert_cmd::Command;

#[test]
fn display_single_file_empty() {
  let mut cmd = Command::cargo_bin("rcat").unwrap();

  cmd.arg("./tests/inputs/empty.txt");
  cmd.assert().success().stdout("");
}

#[test]
fn display_single_file_phrase() {
  let mut cmd = Command::cargo_bin("rcat").unwrap();

  cmd.arg("./tests/inputs/phrase.txt");
  cmd.assert().success().stdout("We cannot be more sensitive to pleasure without being more sensitive to pain - Alan Watts\n");
}

#[test]
fn display_single_file_non_readable() {
  let mut cmd = Command::cargo_bin("rcat").unwrap();

  cmd.arg("./tests/inputs/non_readable.txt");
  cmd.assert().success().stderr("rcat: ./tests/inputs/non_readable.txt: Permission denied (os error 13)\n");
}

#[test]
fn display_single_file_poem() {
  let mut cmd = Command::cargo_bin("rcat").unwrap();

  cmd.arg("./tests/inputs/poem.txt");
  cmd.assert().success().stdout("The Road Not Taken by Robert Frost

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
");
}

#[test]
fn display_all_files() {
  let mut cmd = Command::cargo_bin("rcat").unwrap();

  cmd.arg("./tests/inputs/phrase.txt");
  cmd.arg("./tests/inputs/empty.txt");
  cmd.arg("./tests/inputs/non_readable.txt");
  cmd.arg("./tests/inputs/poem.txt");

  cmd.assert().success().stderr("rcat: ./tests/inputs/non_readable.txt: Permission denied (os error 13)\n");
  cmd.assert().success().stdout("We cannot be more sensitive to pleasure without being more sensitive to pain - Alan Watts
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
");
}

#[test]
fn display_from_stdin() {
  let mut cmd = Command::cargo_bin("rcat").unwrap();

  cmd.write_stdin("Pancakes and apple pie are delicious");

  cmd.assert().success().stdout("Pancakes and apple pie are delicious\n");
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
  cmd.assert().success().stderr("rcat: ./tests/inputs/non_readable.txt: Permission denied (os error 13)\n");
}

