class StatementSet:
    def __init__(self):
        self.statements = {}

    def add(self, statement):
        self.statements[statement.id] = statement

    def latex_code(self, _id):
        if _id not in self.statements:
            return ''
        statement = self.statements[_id]
        # finding parent references in description
        split_desc = statement.description.split()
        num_parents = 0
        for i, token in enumerate(split_desc):
            if token in self.statements:
                num_parents += 1
                parent_statement = self.statements[token]
                split_desc[i] = ' \hyperref[%s:%s]{%s}' % (
                    parent_statement.type, parent_statement.id, parent_statement.id)
        statement.description = ' '.join(split_desc) + r'\par'
        # num parents
        if num_parents == 0:
            num_parents_latex = r'\textbf{Axiom}'
        else:
            num_parents_latex = r'\textbf{%d parent(s)}' % num_parents
        # significance
        if statement.significance.isspace():
            latex_significance = r'{\color{red} No significance?}'
        else:
            latex_significance = r'\textbf{Significance}:%s' % statement.significance
        # complete latex
        latex = r'''
        \begin{%s}
        \label{%s:%s}
        \textbf{%s}\hspace*{0pt}\hfill%s\par
        %s
        %s\par
        \end{%s}
        ''' % (
            statement.type,
            statement.type, statement.id,
            statement.id, num_parents_latex,
            statement.description,
            latex_significance,
            statement.type)
        return latex


class Statement:
    def __init__(self, _id, description, significance, _type):
        self.id = _id
        self.description = description
        self.significance = significance
        self.type = _type
