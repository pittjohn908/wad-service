BEGIN;

-- Insert words
INSERT INTO words (text, language, difficulty) VALUES
('ubiquitous', 'English', 3),
('ephemeral', 'English', 3),
('serendipity', 'English', 4),
('eloquent', 'English', 2),
('paradigm', 'English', 3);

-- Insert definitions
INSERT INTO definitions (word_id, meaning, part_of_speech, usage) VALUES
((SELECT id FROM words WHERE text = 'ubiquitous'), 'Present, appearing, or found everywhere.', 'adjective', 'Mobile phones are now ubiquitous in modern society.'),
((SELECT id FROM words WHERE text = 'ephemeral'), 'Lasting for a very short time.', 'adjective', 'The ephemeral nature of fashion trends makes it hard to keep up.'),
((SELECT id FROM words WHERE text = 'serendipity'), 'The occurrence and development of events by chance in a happy or beneficial way.', 'noun', 'The discovery of penicillin was a case of serendipity.'),
((SELECT id FROM words WHERE text = 'eloquent'), 'Fluent or persuasive in speaking or writing.', 'adjective', 'Her eloquent speech moved the entire audience.'),
((SELECT id FROM words WHERE text = 'paradigm'), 'A typical example or pattern of something; a model.', 'noun', 'This research is an example of the new paradigm in social sciences.');

-- Add additional definitions for words with multiple meanings
INSERT INTO definitions (word_id, meaning, part_of_speech, usage) VALUES
((SELECT id FROM words WHERE text = 'paradigm'), 'A worldview underlying the theories and methodology of a particular scientific subject.', 'noun', 'The paradigm shift in physics brought about by quantum mechanics.');

COMMIT;
