'''
This program compares the take home pay between Salary A and Salary B.

Asssumptions:
- Full marriage allowance
- Plan 2 student loan
- Pension is taken out before tax

'''

# Adjust parameters for salary A
A_salary = 41711
A_pension_rate = 0.05808
A_other_taxable = 489.97

#Adjust paramters for salary B
B_salary = 47000
B_pension_rate = 0.03
B_other_taxable = 5500

#Adjust parameters for below rates and thresholds
marriage_tax_allowance = 1260    #22/23 marriage allowance (make 0 if not applicable)
low_tax_threshold = 12570        #22/23 20% tax threshold
upper_tax_threshold = 50271      #22/23 40% tax threshold
student_loan_threshold = 27295   #22/23 student loan threshold plan 2
student_loan_rate = 0.09         #22/23 student loan rate plan 2
masters_loan_threshold = 21000
masters_loan_rate = 0.06
ni_rate = 0.12                   #22/23 NI rate
ni_threshold = 12576             #22/23 NI threshold
child_tax_threshold = 50000
my_current_child_tax_benefit = 1924

def take_home(salary,pension_rate,other_taxable):
    pension = salary*pension_rate
    gross = salary + other_taxable - pension

    twenty_percent = (gross - low_tax_threshold - marriage_tax_allowance)*0.2
    if gross>upper_tax_threshold:
        forty_percent = (gross - upper_tax_threshold)*0.4
    else:
        forty_percent = 0
    tax = twenty_percent + forty_percent

    student_loan = (gross-student_loan_threshold)*student_loan_rate

    masters_loan = (gross-masters_loan_threshold)*masters_loan_rate

    ni = (gross-ni_threshold)*ni_rate

    if gross>child_tax_threshold:
        child_tax_charge = my_current_child_tax_benefit*(gross-child_tax_threshold)/10000
    else:
        child_tax_charge = 0

    return gross-ni-student_loan-tax-child_tax_charge-masters_loan

A_net = take_home(A_salary,A_pension_rate,A_other_taxable)
B_net = take_home(B_salary,B_pension_rate,B_other_taxable)

print("\n\n")
print("         |   A   |   B   |  +/-  |")
print("----------------------------------")
print("4-Weekly | "+str(round(A_net/13))+"  | "+str(round(B_net/13))+"  | "+"+"+str(round(B_net/13-A_net/13))+"  |")
print("Monthly  | "+str(round(A_net/12))+"  | "+str(round(B_net/12))+"  | "+"+"+str(round(B_net/12-A_net/12))+"  |")
print("Anually  | "+str(round(A_net))+" | "+str(round(B_net))+" | "+"+"+str(round(B_net-A_net))+" |")
print("\n\n")