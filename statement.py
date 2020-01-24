class Statement:
    PARENT_ID_DELIMITER = '@'
    ID_STATEMENT_MAP = {}
    TYPE_DEFINITION = 'definition'

    def __init__(self, _id, description, significance, _type):
        # id validation
        if _id == '':
            raise Exception('Empty statement id found')
        if _id in Statement.ID_STATEMENT_MAP:
            raise Exception('Duplicate statement id found: {}'.format(_id))
        self.id = _id
        # description validation and parsing
        self.description = description
        self.parents = []
        self.children = []
        tokens = self.description.split(Statement.PARENT_ID_DELIMITER)
        if len(tokens) % 2 == 0:
            raise Exception(
                'Invalid parent reference syntax, even number of tokens found in statement with id: {}'.format(_id))
        for i, token in enumerate(tokens):
            # skip even index elements
            if i % 2 == 0:
                continue
            if token in Statement.ID_STATEMENT_MAP:
                parent = Statement.ID_STATEMENT_MAP[token]
                if parent in self.parents:
                    raise Exception('Duplicate parent reference found: {} in statement with id: {}'.format(token, _id))
                self.parents.append(parent)
                parent.children.append(self)
            else:
                raise Exception('Unknown parent reference found: {} in statement with id: {}'.format(token, _id))
        self.significance = significance
        self.type = _type
        # acyclicity check
        if self.cycle_exists():
            raise Exception('Cycle forms by statement with id: {}'.format(_id))
        # add statement to id statement map
        Statement.ID_STATEMENT_MAP[_id] = self

    @staticmethod
    def _cycle_exists(node, origin_id, is_node_the_origin):
        if node.id == origin_id and not is_node_the_origin:
            return True

        for child in node.children:
            if Statement._cycle_exists(child, origin_id, False):
                return True
        return False

    def cycle_exists(self):
        return Statement._cycle_exists(self, self.id, True)

    def latex_format(self):
        tokens = self.description.split(Statement.PARENT_ID_DELIMITER)
        for i, token in enumerate(tokens):
            # skip even index elements
            if i % 2 == 0:
                continue
            # parents and parent reference tokens have corresponding indices
            parent = self.parents[int(i / 2)]
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
