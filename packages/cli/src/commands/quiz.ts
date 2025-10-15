import { writeJson } from '../core/output.js';

export async function runQuiz() {
  const data = {
    questions: [
      { id: 1, q: 'What is phishing?', choices: ['A', 'B', 'C'], answer: 0 },
    ],
    note: 'Stub quiz for MVP',
  };
  const file = writeJson('quiz', 'awareness', data);
  console.log(`Quiz stub written: ${file}`);
}

