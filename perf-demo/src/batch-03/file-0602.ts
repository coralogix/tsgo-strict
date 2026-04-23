import type { Type601 } from '../batch-03/file-0601.js';
export interface Type602 {
  id: 602;
  name: 'File602';
  next: Type601;
}

export function make602(): Type602 {
  return { id: 602, name: 'File602', next: null as unknown as Type601 };
}
