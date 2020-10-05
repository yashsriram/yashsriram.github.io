from utils import *
from pyvis.network import Network
from colorhash import ColorHash


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
            if len(statement.parents) == 0:
                shape = 'dot'
            else:
                shape = 'star'
            color = ColorHash(statement.id).hex
            net.add_node(statement.id,
                         title=statement.description,
                         shape=shape,
                         mass=1 + len(statement.children) * 3 + len(statement.parents) * 2,
                         size=10 + len(statement.children) * 3,
                         borderWidth=1,
                         borderWidthSelected=5,
                         color={
                             'background': '{}{}'.format(color, heat_map(0, len(statement.parents), 5)),
                             'border': color
                         },
                         font={'size': 24, 'color': '#FFFFFF', 'face': 'monospace'})
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
            tokens[i] = r'[\hyperref[statement:%s]{%s}]' % (parent.id, token)
        formatted_description = ''.join(tokens)
        # num parents
        if len(self.parents) == 0:
            latex_proof = r'Axiom.'
        else:
            # proof
            if self.proof.isspace():
                formatted_proof = r'{\color{red} \todo}'
            else:
                formatted_proof = self.proof
            latex_proof = r'''%s''' % formatted_proof
        # description
        if self.description.isspace():
            latex_description = r'{\color{red} \todo}'
        else:
            latex_description = formatted_description
        # significance
        if self.significance.isspace():
            latex_significance = r'{\color{red} \todo}'
        else:
            latex_significance = self.significance
        # parents
        parents_latex = ''
        for parent in self.parents:
            parents_latex += r'\hyperref[statement:%s]{%s}, ' % (parent.id, parent.id)
        # children
        children_latex = ''
        for child in self.children:
            children_latex += r'\hyperref[statement:%s]{%s}, ' % (child.id, child.id)
        if len(self.parents) == 0 and len(self.children) == 0:
            print(self.id)
        # complete latex
        latex = r'''
\addcontentsline{toc}{section}{%s}
\begin{statement}[\textbf{%s}]
\label{statement:%s}\hspace*{0pt}\par
\end{statement}
\textbf{Description}:%s\par
{\color{magenta} \textbf{Significance}:%s\par}
\begin{proof}%s\end{proof}\par
\paragraph{%d parents} %s
\paragraph{%d children} %s
''' % (
            self.id,
            self.id,
            self.id,
            latex_description,
            latex_significance,
            latex_proof,
            len(self.parents), parents_latex,
            len(self.children), children_latex,
            )
        return latex
