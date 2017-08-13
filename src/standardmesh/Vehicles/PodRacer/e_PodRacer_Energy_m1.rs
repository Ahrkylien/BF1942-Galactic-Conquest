subshader "e_PodRacer_Energy_m1_Material0" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular true;
	materialDiffuse 1.0 1.0 1.0;
	materialSpecular 0.337255 0.337255 0.337255;
	materialSpecularPower 12.5;
	twosided true;
	blendSrc sourceAlpha;
	blendDest one;
	alphaTestRef 0.7;
	texture "texture/Effects/e_Podracer_energy_m1";
}

