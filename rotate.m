GG = @(A,B) [ dot(A,B) -norm(cross(A,B)) 0;\
              norm(cross(A,B)) dot(A,B)  0;\
              0              0           1];

FFi = @(A,B) [ A (B-dot(A,B)*A)/norm(B-dot(A,B)*A) cross(B,A) ];

UU = @(Fi,G) Fi*G*inv(Fi);


a=[1 0 0]';
b=[0 1 0]';

 

a=[0.0  1.0 0.0]';
b=[-2.0  0.0   -2.0 ]';

a = a / norm(a);
b = b / norm(b);

a 
b 

U = UU(FFi(a,b), GG(a,b));
norm(U) % is it length-preserving?
U

b_new = U * a; 

b_new

diff = b - b_new; 
diff

