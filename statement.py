class Statement:
    PARENT_ID_DELIMITER = '@'
    ID_STATEMENT_MAP = {}
    TYPE_DEFINITION = 'definition'

    def __init__(self, _id, description, significance, _type):
        """
        Each statement can have some statements as parents.
            Those parents need to be already defined.
            The id of a parent is given in the description near its usage.
        A directed graph formed by such statements should be acyclic
        """
        # id validation
        if _id == '':
            raise Exception('Empty statement id found')
        if _id in Statement.ID_STATEMENT_MAP:
            raise Exception('Duplicate statement id found: {}'.format(_id))
        self.id = _id
        # description validation and parsing
        self.description = description
        self.parents = []
        tokens = self.description.split(Statement.PARENT_ID_DELIMITER)
        if len(tokens) % 2 == 0:
            raise Exception('Invalid parent reference syntax, even number of tokens found in statement with id: {}'.format(_id))
        for i, token in enumerate(tokens):
            # skip even index elements
            if i % 2 == 0:
                continue
            if token in Statement.ID_STATEMENT_MAP:
                parent = Statement.ID_STATEMENT_MAP[token]
                self.parents.append(parent)
            else:
                raise Exception('Unknown parent reference found: {} in statement with id: {}'.format(token, _id))
        self.significance = significance
        self.type = _type
        # add statement to id statement map
        Statement.ID_STATEMENT_MAP[_id] = self

    def latex_format(self):
        tokens = self.description.split(Statement.PARENT_ID_DELIMITER)
        for i, token in enumerate(tokens):
            if token in Statement.ID_STATEMENT_MAP:
                parent = Statement.ID_STATEMENT_MAP[token]
                tokens[i] = r'[\hyperref[%s:%s]{%s}]' % (parent.type, parent.id, parent.id)
        formatted_description = ''.join(tokens) + r'\par'
        # num parents
        if len(self.parents) == 0:
            num_parents_latex = r'\textbf{Axiom}'
        else:
            num_parents_latex = r'\textbf{%d parent(s)}' % len(self.parents)
        # significance
        if self.significance.isspace():
            latex_significance = r'{\color{red} No significance?}'
        else:
            latex_significance = r'\textbf{Significance}:%s' % self.significance
        # complete latex
        latex = r'''
        \begin{%s}
        \label{%s:%s}
        \textbf{%s}\hspace*{0pt}\hfill%s\par
        %s
        %s\par
        \end{%s}
        ''' % (
            self.type,
            self.type, self.id,
            self.id, num_parents_latex,
            formatted_description,
            latex_significance,
            self.type)
        return latex
