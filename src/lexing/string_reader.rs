pub struct StringReader<'a>
{
    source: &'a String,
    index: usize,
    line: usize,
    column: usize
}

impl<'a> StringReader<'a>
{
    pub fn current(&self) -> char
    {
        self.source.chars().nth(self.index).unwrap()
    }

    pub fn source_len(&self) -> usize
    {
        self.source.chars().count()
    }

    pub fn column(&self) -> usize
    {
        self.column
    }

    pub fn line(&self) -> usize
    {
        self.line
    }

    pub fn index(&self) -> usize
    {
        self.index
    }

    pub fn peek(&self, offset: usize) -> Option<char>
    {
        self.source.chars().nth(self.index + offset)
    }

    pub fn advance(&mut self, count: usize) -> String
    {
        if self.index + count > self.source_len()
        {
            panic!("Could not go passed end");
        }

        let new_index = self.index + count;
        let slice = &self.source[self.index..new_index];
        for c in slice.chars()
        {
            if c == '\n'
            {
                self.column = 0;
                self.line += 1;
            }
            else 
            {
                self.column += 1;
            }
        }

        self.index = new_index;

        String::from(slice)
    }

    pub fn at_end(&self) -> bool
    {
        self.index >= self.source_len()
    }

    pub fn create(src: &String) -> StringReader
    {
        StringReader { source: src, index: 0, line: 0, column: 0 }
    }
}