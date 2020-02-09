from utils import *
from pyvis.network import Network


class Statement:
    PARENT_ID_DELIMITER = '@'
    ID_STATEMENT_MAP = {}

    def __init__(self, _id, description, significance, proof):
        # id validation
        if _id == '':
            raise Exception('Empty statement id found')
        if _id in Statement.ID_STATEMENT_MAP:
            raise Exception('Duplicate statement id found: {}'.format(_id))
        id_case_combinations = case_combinations(_id)
        for combination in id_case_combinations:
            if combination in Statement.ID_STATEMENT_MAP:
                raise Exception(
                    'Duplicate case combination of statement id found: {} for id: {}'.format(combination, _id))
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
            # if parent found in some case combination it is valid
            token_case_combinations = case_combinations(token)
            parent_found = False
            for combination in token_case_combinations:
                if combination in Statement.ID_STATEMENT_MAP:
                    parent = Statement.ID_STATEMENT_MAP[combination]
                    if parent in self.parents:
                        raise Exception(
                            'Duplicate parent reference found: {} in statement with id: {}'.format(token, _id))
                    self.parents.append(parent)
                    parent.children.append(self)
                    parent_found = True
                    break
            if not parent_found:
                raise Exception('Unknown parent reference found: {} in statement with id: {}'.format(token, _id))
        self.significance = significance
        if len(self.parents) == 0:
            self.type = 'axiom'
        else:
            self.type = 'theorem'
        self.proof = proof
        # acyclicity check
        if self.cycle_exists():
            raise Exception('Cycle forms by statement with id: {}'.format(_id))
        # add statement to id statement map
        Statement.ID_STATEMENT_MAP[_id] = self

    @staticmethod
    def html_dag_format(filename):
        net = Network(width='100%', height='100%', directed=True, bgcolor='#000000')
        for _id, statement in Statement.ID_STATEMENT_MAP.items():
            net.add_node(statement.id,
                         title=statement.description,
                         shape='star',
                         mass=1 + len(statement.children) * 3 + len(statement.parents) * 2,
                         size=10 + len(statement.children) * 3,
                         borderWidth=1,
                         borderWidthSelected=5,
                         color={'background': '#FFFFFF10', 'border': '#FFFFFFAA'},
                         font={'size': 12, 'color': '#FFFFFF', 'face': 'monospace'})
            for parent in statement.parents:
                net.add_edge(parent.id, _id, arrowStrikethrough=False)
        net.save_graph(filename)

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
            tokens[i] = r'[\hyperref[%s:%s]{%s}]' % (parent.type, parent.id, token)
        formatted_description = ''.join(tokens) + r'\par'
        # num parents
        if len(self.parents) == 0:
            num_parents_latex = r''
            latex_proof = ''
        else:
            num_parents_latex = r'\textbf{%d parent(s)}' % len(self.parents)
            # proof
            if self.proof.isspace():
                formatted_proof = r'{\color{red} TODO}\par'
            else:
                formatted_proof = self.proof
            latex_proof = r'''
            \begin{proof}
            %s
            \end{proof}
            ''' % formatted_proof
        # description
        if self.description.isspace():
            latex_description = r'{\color{red} TODO}\par'
        else:
            latex_description = formatted_description
        # significance
        if self.significance.isspace():
            latex_significance = r'{\color{red} No significance?}'
        else:
            latex_significance = r'\textbf{Significance}:%s' % self.significance
        # complete latex
        latex = r'''
        \begin{%s}[\textbf{%s}]
        \label{%s:%s}
        \hspace*{0pt}\hfill%s\par
        %s
        %s\par
        \end{%s}
        %s
        ''' % (
            self.type, self.id,
            self.type, self.id,
            num_parents_latex,
            latex_description,
            latex_significance,
            self.type,
            latex_proof)
        return latex
