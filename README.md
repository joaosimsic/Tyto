# Tyto

DSL (Domain Specific Language): 

Example:
state Pendente {
    on Pagar -> Pago;
    on Cancelar -> Cancelado;
}

state Pago {
    data { transaction_id: String }
    on Enviar -> Enviado;
    on Reembolsar -> Reembolsado;
}

state Enviado {
    terminal; 
}
