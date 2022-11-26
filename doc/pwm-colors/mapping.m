max_val = 65535;
i = 1:100:max_val;

r = exp(min((i/8000 + 7.0), log(max_val)));
g = exp(min((i/8000 + 5.0), log(max_val)));
b = exp(min((i/8000 + 3.0), log(max_val)));

plot(i, r, "r", "linewidth", 5);
xlabel ("Input");
ylabel ("PWM Duty");
hold on;
plot(i, g, "g", "linewidth", 5);
h = plot(i, b, "b", "linewidth", 5);

title ("ADC to PWM duty mapping");

waitfor(h);